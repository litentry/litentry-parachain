import net from "net";

function checkPort(port) {
  return new Promise((resolve, reject) => {
    const server = net.createServer();

    server.once("error", (err) => {
      if (err.code === "EADDRINUSE") {
        // Port is already in use
        resolve(false);
      } else {
        // Other error, reject the promise
        reject(err);
      }
    });

    server.once("listening", () => {
      // Port is available
      server.close();
      resolve(true);
    });

    server.listen(port, "127.0.0.1");
  });
}

export async function isPortAvailable(port) {
  try {
    return await checkPort(port);
  } catch (e) {
    return false;
  }
}
