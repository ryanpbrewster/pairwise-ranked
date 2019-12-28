#!/usr/bin/env python3

from http.server import HTTPServer, SimpleHTTPRequestHandler

PORT = 8080

class Handler(SimpleHTTPRequestHandler):
    pass

Handler.extensions_map['.shtml'] = 'text/html'
Handler.extensions_map['.wasm'] = 'application/wasm'

httpd = HTTPServer(('', PORT), Handler)
print(f"serving at port {PORT}")
httpd.serve_forever()
