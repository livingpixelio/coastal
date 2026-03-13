import React from "react";
import ReactDOM from "react-dom/client";
import Database from "@tauri-apps/plugin-sql";
const db = await Database.load("sqlite:localdata.db");

import App from "./App";
import { TauriFileSystemStorageAdapter } from "./automerge-repo-storage-tauri-fs";

import { AnyDocumentId, DocHandle, Repo } from "@automerge/automerge-repo";
const repo = new Repo({
  network: [],
  storage: new TauriFileSystemStorageAdapter(),
});

const URL = "automerge:21FFsqfQ9stqAuFQxkUh1RCdXjmj" as AnyDocumentId;

let handle: DocHandle<{ foo: "bar"; counter: number }> = await repo.find(URL);
if (!handle) {
  handle = repo.create({ foo: "bar", counter: 0 });
  console.log(handle.url);
}
handle.change((doc) => doc.counter++);
console.log(handle.doc());

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App db={db} />
  </React.StrictMode>,
);
