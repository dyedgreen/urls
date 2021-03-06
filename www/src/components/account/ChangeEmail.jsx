import { h } from "preact";
import { useState } from "preact/hooks";
import { graphql, useMutation } from "picoql";

import TextInput from "@app/TextInput";
import Button from "@app/Button";
import Notice from "@app/Notice";

export default function ChangeEmail({}) {
  const [email, setEmail] = useState("");
  const [confirmEmail, setConfirmEmail] = useState("");

  const [error, setError] = useState(null);
  const [notice, setNotice] = useState(null);

  const { commit, inFlight } = useMutation(graphql`
    mutation ChangeEmailMutation($email: String!) {
      updateUser(input: {email: $email}) {
        id
        email
      }
    }
  `, {
    onCommit: ({ updateUser }) => {
      setEmail("");
      setConfirmEmail("");
      setError(null);
      setNotice(`Email changed to ${updateUser?.email}`);
    },
    onError: ([{message}]) => {
      setError(`Failed to change email: ${message}`);
      setNotice(null);
    },
  });

  const canSubmit = !inFlight && email.trim().length > 0 && email === confirmEmail;
  const submit = e => {
    e.preventDefault();
    if (canSubmit)
      commit({ email: email.trim() });
  };

  return <form onSubmit={submit}>
    {error && <Notice message={error} type="error" style="mb-2" />}
    {notice && <Notice message={notice} style="mb-2" />}
    <TextInput
      label="New email address"
      placeholder="ada.lovelace@urls.fyi"
      type="email"
      value={email}
      onChange={setEmail}
      style="mb-2"
    />
    <TextInput
      label="Confirm email address"
      placeholder="ada.lovelace@urls.fyi"
      type="email"
      value={confirmEmail}
      onChange={setConfirmEmail}
      style="mb-2"
    />
    <Button
      title="Update"
      onClick={submit}
      disabled={!canSubmit}
      loading={inFlight}
      style="w-full"
    />
  </form>;
}
