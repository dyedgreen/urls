import { render, h } from "preact";
import Test from "@app/Test";
import ErrorBoundary from "@app/ErrorBoundary";

function App() {
  return <ErrorBoundary>
    <Test />
  </ErrorBoundary>;
}

render(<App />, document.getElementById("login"));
