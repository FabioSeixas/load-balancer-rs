import http from "http"

http.createServer(async (req, res) => {
  const server = req.headers.host
  res.end(JSON.stringify({ message: `ok from ${server}` }))
}).listen(process.argv[2], () => console.log("server is ready at ", process.argv[2]))

