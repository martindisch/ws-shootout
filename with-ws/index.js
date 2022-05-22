import { WebSocketServer } from "ws";

const clients = new Set();

const wss = new WebSocketServer({ port: 8080 });

wss.on("connection", (ws) => {
  clients.add(ws);

  ws.on("close", () => clients.delete(ws));
});

const periodicUpdate = () => {
  const date = new Date();
  const fullDate = date.toString();
  const time = date.toLocaleTimeString("de-CH");

  console.log(`[${time}] Sending date to ${clients.size} clients`);
  clients.forEach((ws) => ws.send(fullDate));

  setTimeout(periodicUpdate, 1000);
};

periodicUpdate();
