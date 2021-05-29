import { h } from "preact";
import { useState } from "preact/hooks";
import { graphql, useMutation } from "picoql";

import TextInput from "@app/TextInput";
import Button from "@app/Button";
import Notice from "@app/Notice";

export default function ChangeName({ currentName }) {
  const [name, setName] = useState(currentName);

  const [error, setError] = useState(null);
  const [notice, setNotice] = useState(null);

  const { commit, inFlight } = useMutation(graphql`
    mutation ChangeName($name: String!) {
      updateUser(input: {name: $name}) {
        id
        user {
          id
          name
        }
      }
    }
  `, {
    onCommit: ({ updateUser }) => {
      setNotice(`Name changed to ${updateUser?.user?.name}`);
      setName(updateUser?.user?.name);
      setError(null);
    },
    onError: ([{message}]) => {
      setError(`Failed to change email: ${message}`);
      setNotice(null);
    },
  });

  const canSubmit = !inFlight && name.trim().length > 0;
  const submit = e => {
    e.preventDefault();
    if (canSubmit)
      commit({ name: name.trim() });
  };

  return <form onSubmit={submit}>
    {error && <Notice message={error} type="error" style="mb-2" />}
    {notice && <Notice message={notice} style="mb-2" />}
    <TextInput
      label="Public profile name"
      placeholder="Ada Lovelace"
      value={name}
      onChange={setName}
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
