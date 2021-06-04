import { h, Component } from "preact";
import Button from "@app/Button";

function reload() {
  window.location.href = window.location.href;
}

export function errorToString(err) {
  const obj = {};
  for (const key of Object.getOwnPropertyNames(err)) {
    obj[key] = err[key];
  }
  return JSON.stringify(obj);
};

function ErrorInfo({ error }) {
  return <div class="w-full flex justify-center p-8">
    <div class="w-full max-w-3xl bg-red-500 rounded-lg p-4">
      <h1 class="text-white text-xl font-semibold">Error</h1>
      <p class="text-red-100">Sorry, something went wrong.</p>
      <pre class="text-white bg-red-900 rounded-lg p-4 mt-2 whitespace-pre-wrap break-all">
        {errorToString(error)}
      </pre>

      <Button title="Reload Page" onClick={reload} type="flat" style="w-full mt-2" />
    </div>
  </div>;
}

export default class ErrorBoundary extends Component {
  constructor() {
    super();
    this.state = { error: false };
  }

  componentDidCatch(error) {
    console.error(error);
    this.setState({ error });
  }

  render({ children }, { error }) {
    if (error !== false) {
      return <ErrorInfo error={error} />;
    }
    return children;
  }
}
