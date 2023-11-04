import * as http from "node:http";
import { dataToNumberArray, getRouteAndArgs } from "./util.mjs";

const ipcHandlePath = process.env["IPC_HANDLE_PATH"];

const server = http.createServer();

server.once("request", async (req, res) => {
  const r = getRouteAndArgs(req.url);
  if (req.method === "POST") {
    // handle data from request
    for await (const chunk of req) {
      //  chunk to UInt8Array
      const s = JSON.stringify({ id: 2, data: dataToNumberArray(chunk) });
      await new Promise((resolve) => res.write(s, resolve));
    }
    res.end();
  } else {
    res.end("only POST method is allowed");
  }
  server.close();
});

server.listen(ipcHandlePath || 3000);
