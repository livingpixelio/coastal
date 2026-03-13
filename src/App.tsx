import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Database, { QueryResult } from "@tauri-apps/plugin-sql";

interface Props {
  db: Database;
}

function App({ db }: Props) {
  const [name, setName] = useState("");
  const [saved, setSaved] = useState<any[]>([]);

  async function greet() {
    db.execute('INSERT into feeds (slug, value) VALUES ($1, $2)', [name, name]);
  }

  useEffect(() => {
    db.select('SELECT * FROM feeds WHERE 1=1').then((res: any[]) => {
      setSaved(res);
    });
  }, []);

  return (
    <main className="container">
      <ul>
        {saved.map(item => <li key={item.slug}>{item.value}</li>)}
      </ul>

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
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
    </main>
  );
}

export default App;
