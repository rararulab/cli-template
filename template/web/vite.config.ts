import { defineConfig } from "vite";
import fs from "fs";
import path from "path";

export default defineConfig({
  base: "/cli-template/",
  server: {
    fs: { allow: ["."] },
  },
  plugins: [
    {
      name: "serve-mdbook",
      configureServer(server) {
        server.middlewares.use("/cli-template/docs", (req, res, next) => {
          const url = req.url === "/" || req.url === "" ? "/index.html" : req.url;
          const filePath = path.resolve(__dirname, "docs/book", url!.slice(1));
          try {
            const content = fs.readFileSync(filePath);
            const ext = path.extname(filePath);
            const types: Record<string, string> = {
              ".html": "text/html",
              ".css": "text/css",
              ".js": "application/javascript",
              ".json": "application/json",
              ".png": "image/png",
              ".svg": "image/svg+xml",
              ".woff2": "font/woff2",
            };
            res.setHeader("Content-Type", types[ext] || "application/octet-stream");
            res.end(content);
          } catch {
            next();
          }
        });
      },
    },
  ],
});
