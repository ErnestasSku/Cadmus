import "./style.css";
import App from "./App.svelte";
import Editor from "./lib/Editor.svelte";

const app = new Editor({
  target: document.getElementById("app"),
});

export default app;
