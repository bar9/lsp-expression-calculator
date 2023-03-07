import React from "react";
import ReactDOM from "react-dom";

import {ReactMonacoEditor} from "./ReactMonacoEditor";
import {createRoot} from "react-dom/client";

function App() {
  return (
    <ReactMonacoEditor
      // height="90vh"
      // defaultLanguage="javascript"
      defaultCode="// some comment"
    />
  );
}

const rootElement = document.getElementById("root");
// @ts-ignore
const root = createRoot(rootElement);
root.render(<App/>);