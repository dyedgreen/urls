import { render, h } from "preact";
import { useState } from "preact/hooks";
import { graphql, useMutation } from "picoql";

import ErrorBoundary from "@app/ErrorBoundary";
import TextInput from "@app/TextInput";
import Button from "@app/Button";
import Notice from "@app/Notice";

function Login() {
  const [email, setEmail] = useState("");
  const [code, setCode] = useState("");
  const [showCode, setShowCode] = useState(false);

  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [notice, setNotice] = useState(null);

  const request = useMutation(graphql`
    mutation RequestLogin($email: String!) {
      requestLogin(email: $email) {
        ok
      }
    }
  `);
  const commitRequest = () => {
    setLoading(true);
    request
      .commit({ email })
      .then(() => {
        setNotice(`A login code was sent to ${email}.`);
        setError(null);
        setShowCode(true);
        setLoading(false);
      })
      .catch((errors) => {
        setError(`Could not request code: ${errors[0].message}`);
        setLoading(false);
      });
  };

  const login = useMutation(graphql`
    mutation Login($email: String!, $code: String!) {
      login(email: $email, token: $code)
    }
  `);
  const commitLogin = () => {
    setLoading(true);
    login
      .commit({ email, code })
      .then(({login}) => {
        // redirect to home page, so no need to reset state ...
        document.cookie = `${window.__auth_cookie}=${login};path=/;max-age=604800`;
        window.location.href = "/";
      })
      .catch((errors) => {
        setError(`Failed to log in: ${errors[0].message}`);
        setLoading(false);
      });
  };

  const buttonAction = showCode ? commitLogin : commitRequest;

  return <ErrorBoundary>
    <div class="w-full flex justify-center p-8">
      <div class="w-full max-w-md bg-white shadow rounded-lg p-4">
        <h1 class="text-2xl font-semibold">Login</h1>
        {notice && <Notice message={notice} type="info" style="mt-2" />}
        {error && <Notice message={error} type="error" style="mt-2" />}

        <TextInput
          label="Email Address"
          placeholder="ada.lovelace@urls.fyi"
          style="mt-2"
          value={email}
          onChange={setEmail}
        />
        {
          showCode &&
          <TextInput
            label="Login Code"
            placeholder="N8yDO3aKiIyT"
            style="mt-2"
            value={code}
            onChange={setCode}
          />
        }

        <Button
          title={showCode ? "Login" : "Request Code"}
          onClick={buttonAction}
          style="mt-2 w-full"
          disabled={loading}
          loading={loading}
        />
        <button onClick={() => setShowCode(!showCode)} class="w-full mt-2 text-center text-blue-500">
          {showCode ? "I need a login code" : "I already have a login code"}
        </button>
      </div>
    </div>
  </ErrorBoundary>;
}

render(<Login />, document.getElementById("login"));
