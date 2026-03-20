import { useState } from "react";
import "./App.css";
import { invoke } from "@tauri-apps/api/core";

interface Props {
  url: string;
  counter: number;
}

function App({ url, counter }: Props) {
  const [name, setName] = useState("");
  const [saved, setSaved] = useState<any[]>([]);

  const handleCreateSharingCode = () => {
    invoke("create_share", { url }).then(console.log);
  };

  return (
    <main className="container">
      <ul>
        <li>URL: {url}</li>
        <li>COUNTER: {counter}</li>
      </ul>

      <div>
        <button type="button" onClick={handleCreateSharingCode}>
          Create code
        </button>
      </div>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          // greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
    </main>
  );
}

export default App;
