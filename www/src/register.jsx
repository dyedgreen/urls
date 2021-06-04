import { render, h } from "preact";
import { useState } from "preact/hooks";
import { graphql, useMutation } from "picoql";

import ErrorBoundary from "@app/ErrorBoundary";
import TextInput from "@app/TextInput";
import Button from "@app/Button";
import Notice from "@app/Notice";

function Register() {
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [code, setCode] = useState("");

  const [error, setError] = useState(null);

  const { commit, inFlight } = useMutation(graphql`
    mutation RegisterMutation($input: NewUserInput!, $code: String!) {
      registerUser(input: $input, token: $code) {
        id
      }
    }
  `, {
    onCommit: () => window.location.href = "/login",
    onError: ([{message}]) => setError(`Failed to register: ${message.replace(/\n/g, ", ")}`),
  });
  const submit = e => {
    e.preventDefault();
    commit({
      input: { name, email },
      code,
    });
  };

  return (
    <div class="w-full flex justify-center p-8">
      <form class="w-full max-w-md bg-white dark:bg-gray-800 shadow rounded-lg p-4" onSubmit={submit}>
        <h1 class="text-2xl font-semibold">Register</h1>
        {error && <Notice message={error} type="error" style="mt-2" />}

        <TextInput
          label="Your name"
          placeholder="Ada Lovelace"
          style="mt-2"
          value={name}
          onChange={setName}
        />
        <TextInput
          label="Email Address"
          placeholder="ada.lovelace@urls.fyi"
          style="mt-2"
          value={email}
          onChange={setEmail}
        />
        <TextInput
          label="Invitation Code"
          placeholder="Your invitation code"
          style="mt-2"
          value={code}
          onChange={setCode}
        />

        <Button
          title="Register"
          onClick={submit}
          style="mt-2 w-full"
          disabled={inFlight}
          loading={inFlight}
        />
      </form>
    </div>
  );
}

render(<ErrorBoundary><Register /></ErrorBoundary>, document.getElementById("register"));
