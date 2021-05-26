import { render, h } from "preact";
import { useState } from "preact/hooks";
import { graphql, useMutation } from "picoql";

import Test from "@app/Test";
import ErrorBoundary from "@app/ErrorBoundary";
import TextInput from "@app/TextInput";
import Button from "@app/Button";
import Notice from "@app/Notice";

function App() {
  const [email, setEmail] = useState("");
  const [error, setError] = useState(null);
  const [showCode, setShowCode] = useState(false);

  const request = useMutation(graphql`
    mutation RequestLogin($email: String!) {
      requestLogin(email: $email) {
        ok
      }
    }
  `);
  const requestCode = () => {
    request
      .commit({ email })
      .then(() => {
        setError(null);
        setShowCode(true);
      })
      .catch(() => setError("Could not request code."));
  };

  return <ErrorBoundary>
    <div class="w-full flex justify-center p-8">
      <div class="w-full max-w-md bg-white rounded-lg p-4">
        <h1 class="text-2xl font-semibold">Login</h1>
        {error && <Notice message={error} type="error" style="mt-2" />}

        <TextInput
          label="Email Address"
          placeholder="ada.lovelace@urls.fyi"
          style="mt-2"
          value={email}
          onChange={setEmail}
        />
        {showCode && <TextInput label="Login Code" placeholder="N8yDO3aKiIyT" style="mt-2" />}

        <Button
          title={showCode ? "Login" : "Request Code"}
          onClick={requestCode}
          style="mt-2 w-full"
        />
        <button onClick={() => setShowCode(!showCode)} class="w-full mt-2 text-center text-blue-500">
          {showCode ? "I need a login code" : "I already have a login code"}
        </button>
      </div>
    </div>
  </ErrorBoundary>;
}

render(<App />, document.getElementById("login"));
