import http from "http"
import { exec } from "child_process"

for (let i = 0; i < 10; i++) {
  exec(`node server.js 300${i}`)
}

