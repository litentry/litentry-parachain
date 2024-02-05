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
		await checkPort(port);
		return true;
	} catch (e) {
		return "port is busy try another one" + e.message;
	}
}
