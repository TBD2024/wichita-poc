import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("prompt_enhancer", { prompt: name }));
  }

  return (
    <div className="container">
      <h1>Psyborg</h1>
      <h4>"The easiest way to boost your productivity when using generative AI."</h4>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter prompt..."
        />
        <button type="submit">Enhance</button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
