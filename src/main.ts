import "./app.css";

import Kiosk from "./routes/Kiosk.svelte";
import Kitchen from "./routes/Kitchen.svelte";
import Admin from "./routes/Admin.svelte";

const urlParams = new URLSearchParams(window.location.search);
const mode = urlParams.has("kitchen")
  ? "kitchen"
  : urlParams.has("admin")
  ? "admin"
  : "kiosk";

if (mode === "kitchen") {
  new Kitchen({ target: document.getElementById("app")! });
} else if (mode === "admin") {
  new Admin({ target: document.getElementById("app")! });
} else {
  new Kiosk({ target: document.getElementById("app")! });
}
