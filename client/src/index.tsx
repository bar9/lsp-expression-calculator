import React from "react";
import ReactDOM from "react-dom";

import Editor from "@monaco-editor/react";

function App() {
  return (
    <Editor
      height="90vh"
      defaultLanguage="javascript"
      defaultValue="// some comment"
    />
  );
}

const rootElement = document.getElementById("root");
ReactDOM.render(<App />, rootElement);