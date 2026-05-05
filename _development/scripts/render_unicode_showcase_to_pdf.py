"""
Render the UniWorld Unicode showcase markdown to HTML and PDF.

Flow: Markdown -> HTML (markdown lib) -> PDF via Edge/Chrome DevTools Protocol.
Uses the system browser with full rendering (no extra pip libs for PDF).
Full Unicode font rendering because the browser uses system fonts.

Usage (from repo root):
  python _development/scripts/render_unicode_showcase_to_pdf.py

Requirements (pip): markdown only.

What this showcases:
- HTML: proves the document can be stored, served, and rendered as UTF-8
  with correct script/font behavior in any browser or viewer.
- PDF: same content in a portable format, rendered by the same engine,
  so fonts and bidi are consistent. Together they demonstrate the
  pipeline UniWorld targets: author -> store -> render complex Unicode
  end-to-end. The library itself is validated by unit/integration tests;
  this script validates the ecosystem (editors, browsers, fonts) for
  that content.

Output:
  docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.html
  docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.pdf
"""

import base64
import json
import os
import shutil
import subprocess
import sys
import time
import urllib.request
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
MD_PATH = REPO_ROOT / "docs" / "UniWorld_Unicode_Showcase_TEST_OUTPUT.md"
HTML_PATH = REPO_ROOT / "docs" / "UniWorld_Unicode_Showcase_TEST_OUTPUT.html"
PDF_PATH = REPO_ROOT / "docs" / "UniWorld_Unicode_Showcase_TEST_OUTPUT.pdf"

# Minimal CSS. No print-specific rules -- let the browser defaults handle paging.
HTML_TEMPLATE = """<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>UniWorld Unicode Showcase (TEST OUTPUT)</title>
  <style>
    body {{
      font-family: "Segoe UI", "Noto Sans", "Noto Sans CJK SC",
                   "Noto Sans Arabic", "Noto Sans Hebrew", system-ui, sans-serif;
      line-height: 1.6;
      max-width: 800px;
      margin: 2em auto;
      padding: 0 1em;
      color: #1a1a1a;
    }}
    h1, h2, h3 {{ margin-top: 1.5em; }}
    pre, code {{ font-family: Consolas, "Liberation Mono", monospace; }}
    pre {{ white-space: pre-wrap; word-wrap: break-word; overflow-wrap: break-word; }}
    blockquote {{
      border-left: 4px solid #ccc;
      margin-left: 0;
      padding-left: 1em;
      color: #444;
    }}
    blockquote code {{
      white-space: normal;
      word-break: break-word;
      overflow-wrap: anywhere;
    }}
  </style>
</head>
<body>
{body}
</body>
</html>
"""


def _find_browser() -> Path | None:
    """Return path to Edge or Chrome/Chromium executable, or None."""
    if sys.platform == "win32":
        pf86 = Path(os.environ.get("ProgramFiles(x86)", "C:\\Program Files (x86)"))
        pf = Path(os.environ.get("ProgramFiles", "C:\\Program Files"))
        candidates = [
            pf86 / "Google" / "Chrome" / "Application" / "chrome.exe",
            pf / "Google" / "Chrome" / "Application" / "chrome.exe",
            pf86 / "Microsoft" / "Edge" / "Application" / "msedge.exe",
            pf / "Microsoft" / "Edge" / "Application" / "msedge.exe",
        ]
        for p in candidates:
            if p.exists():
                return p
        return None
    for name in ("google-chrome", "google-chrome-stable", "chromium", "chromium-browser", "chrome"):
        exe = shutil.which(name)
        if exe:
            return Path(exe)
    return None


def _cdp_send(ws_url: str, method: str, params: dict | None = None) -> dict:
    """Send a CDP command over HTTP (using the /json/protocol HTTP endpoint fallback)."""
    # We use the HTTP endpoint rather than websockets to avoid extra dependencies.
    # This won't work for CDP -- we need websockets. Fall back to a simple approach.
    raise NotImplementedError


def _print_pdf_via_cdp(browser: Path, html_uri: str, pdf_path: Path) -> bool:
    """Launch browser with remote debugging, navigate to page, use Page.printToPDF."""
    import socket
    import struct

    # Find a free port for remote debugging.
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(("127.0.0.1", 0))
        port = s.getsockname()[1]

    user_data = Path(os.environ.get("TEMP", "/tmp")) / f"uniworld_cdp_{port}"

    cmd = [
        str(browser),
        "--headless=new",
        f"--remote-debugging-port={port}",
        f"--user-data-dir={user_data}",
        "--no-first-run",
        "--disable-extensions",
        "--disable-default-apps",
        "about:blank",
    ]
    proc = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    try:
        # Wait for DevTools to be ready.
        ws_url = None
        for _ in range(30):
            time.sleep(0.5)
            try:
                resp = urllib.request.urlopen(f"http://127.0.0.1:{port}/json/version")
                info = json.loads(resp.read())
                ws_url = info.get("webSocketDebuggerUrl")
                break
            except Exception:
                continue

        if not ws_url:
            print("Error: could not connect to browser DevTools", file=sys.stderr)
            return False

        # Get page target.
        resp = urllib.request.urlopen(f"http://127.0.0.1:{port}/json/list")
        targets = json.loads(resp.read())
        page_ws = None
        for t in targets:
            if t.get("type") == "page":
                page_ws = t.get("webSocketDebuggerUrl")
                break

        if not page_ws:
            print("Error: no page target found", file=sys.stderr)
            return False

        # Use websocket (stdlib has no websocket client, so we do a minimal one).
        import hashlib
        import struct as st

        sock = socket.create_connection(("127.0.0.1", port))

        # Parse ws URL for path.
        # ws://127.0.0.1:PORT/devtools/page/XXXXX
        ws_path = page_ws.split(f":{port}", 1)[1]

        # WebSocket handshake.
        ws_key = base64.b64encode(os.urandom(16)).decode()
        handshake = (
            f"GET {ws_path} HTTP/1.1\r\n"
            f"Host: 127.0.0.1:{port}\r\n"
            f"Upgrade: websocket\r\n"
            f"Connection: Upgrade\r\n"
            f"Sec-WebSocket-Key: {ws_key}\r\n"
            f"Sec-WebSocket-Version: 13\r\n"
            f"\r\n"
        )
        sock.sendall(handshake.encode())

        # Read handshake response (just consume until \r\n\r\n).
        hs_resp = b""
        while b"\r\n\r\n" not in hs_resp:
            hs_resp += sock.recv(4096)

        if b"101" not in hs_resp:
            print("Error: WebSocket handshake failed", file=sys.stderr)
            sock.close()
            return False

        msg_id = 0

        def ws_send(method: str, params: dict | None = None) -> dict:
            nonlocal msg_id
            msg_id += 1
            payload = json.dumps({"id": msg_id, "method": method, "params": params or {}})
            data = payload.encode("utf-8")
            # Build WebSocket frame (masked, text).
            frame = bytearray()
            frame.append(0x81)  # FIN + text
            mask_key = os.urandom(4)
            length = len(data)
            if length < 126:
                frame.append(0x80 | length)
            elif length < 65536:
                frame.append(0x80 | 126)
                frame.extend(st.pack("!H", length))
            else:
                frame.append(0x80 | 127)
                frame.extend(st.pack("!Q", length))
            frame.extend(mask_key)
            masked = bytearray(b ^ mask_key[i % 4] for i, b in enumerate(data))
            frame.extend(masked)
            sock.sendall(bytes(frame))

            # Read response frames until we get our id back.
            while True:
                msg = _ws_recv(sock)
                if msg is None:
                    return {}
                parsed = json.loads(msg)
                if parsed.get("id") == msg_id:
                    return parsed
                # else it's an event, keep reading

        def _ws_recv(s: socket.socket) -> str | None:
            """Read one WebSocket text frame."""
            header = _recv_exact(s, 2)
            if not header:
                return None
            opcode = header[0] & 0x0F
            masked = bool(header[1] & 0x80)
            length = header[1] & 0x7F
            if length == 126:
                length = st.unpack("!H", _recv_exact(s, 2))[0]
            elif length == 127:
                length = st.unpack("!Q", _recv_exact(s, 8))[0]
            if masked:
                mask_key = _recv_exact(s, 4)
            payload = _recv_exact(s, length)
            if masked:
                payload = bytes(b ^ mask_key[i % 4] for i, b in enumerate(payload))
            if opcode == 1:  # text
                return payload.decode("utf-8")
            return None

        def _recv_exact(s: socket.socket, n: int) -> bytes:
            buf = b""
            while len(buf) < n:
                chunk = s.recv(n - len(buf))
                if not chunk:
                    return buf
                buf += chunk
            return buf

        # Enable Page domain.
        ws_send("Page.enable")

        # Navigate to the HTML file.
        nav_result = ws_send("Page.navigate", {"url": html_uri})

        # Wait for page to fully load and render (emoji/complex glyphs need time).
        time.sleep(5)

        # Print to PDF with full page rendering.
        pdf_result = ws_send("Page.printToPDF", {
            "landscape": False,
            "displayHeaderFooter": False,
            "printBackground": True,
            "preferCSSPageSize": False,
            "paperWidth": 8.5,
            "paperHeight": 11.0,
            "marginTop": 0.75,
            "marginBottom": 0.75,
            "marginLeft": 0.75,
            "marginRight": 0.75,
        })

        sock.close()

        if "result" in pdf_result and "data" in pdf_result["result"]:
            pdf_bytes = base64.b64decode(pdf_result["result"]["data"])
            pdf_path.write_bytes(pdf_bytes)
            return True
        else:
            print(f"Error: Page.printToPDF failed: {pdf_result}", file=sys.stderr)
            return False

    finally:
        proc.terminate()
        try:
            proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            proc.kill()
        # Clean up temp user data dir.
        import shutil as sh
        try:
            sh.rmtree(user_data, ignore_errors=True)
        except Exception:
            pass


def main() -> int:
    if not MD_PATH.exists():
        print(f"Error: Markdown not found: {MD_PATH}", file=sys.stderr)
        return 1

    try:
        import markdown
    except ImportError:
        print("Error: markdown not installed. Run: pip install markdown", file=sys.stderr)
        return 1

    text = MD_PATH.read_text(encoding="utf-8")
    html_body = markdown.markdown(text, extensions=["extra"])
    html_doc = HTML_TEMPLATE.format(body=html_body)

    HTML_PATH.parent.mkdir(parents=True, exist_ok=True)
    HTML_PATH.write_text(html_doc, encoding="utf-8")
    print(f"Wrote HTML: {HTML_PATH}")

    browser = _find_browser()
    if not browser:
        print(
            "Error: No Edge or Chrome found.",
            file=sys.stderr,
        )
        return 1

    print(f"Using browser: {browser}")
    html_uri = HTML_PATH.resolve().as_uri()

    if _print_pdf_via_cdp(browser, html_uri, PDF_PATH):
        print(f"Wrote PDF:  {PDF_PATH}")
        return 0
    else:
        print("Error: PDF generation failed.", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
