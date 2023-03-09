import "./App.css";
import {ReactMonacoEditor} from "./ReactMonacoEditor";
const Link = (props: JSX.IntrinsicElements['a']) => (
  <a
    className="text-pink-500 underline hover:no-underline dark:text-pink-400"
    {...props}
  />
);

const code = `{
  "type": "SUM_ROW",
  "name": "CASH_FLOW_FROM_OPERATIONS",
  "title": "Cash Flow From Operations",
  "unit": "MONEY",
  "formula": "NET_EARNINGS + ADDITIONS_TO_CASH - SUBTRACTIONS_FROM_CASH"
}
`

export default function App() {
  return (
    <>
    <div className="mx-auto my-8 mt-10 w-8/12 rounded border border-gray-200 p-4 shadow-md dark:border-neutral-600 dark:bg-neutral-800 dark:shadow-none">
      <h1 className="mb-4 text-4xl">A configuration screen in a web app</h1>
      <p className="my-4">
        <em>We're developers. <br/> Most of us don't like configurators, wizards, complicated UIs. <br/>We prefer code and the little helpers inside our IDE.<br/>Maybe our customers prefer this too, if we show them.</em>
      </p>

    </div>
    <div className="mx-auto my-8 mt-10 w-8/12">
      <ReactMonacoEditor
        defaultLanguage="json"
        defaultCode={code}
      />
    </div>
  </>
  );
}
