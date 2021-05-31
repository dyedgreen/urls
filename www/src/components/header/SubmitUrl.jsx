import { h } from "preact";
import { useState } from "preact/hooks";
import { graphql, useMutation } from "picoql";

import TextInput from "@app/TextInput";
import Button from "@app/Button";
import Notice from "@app/Notice";

export default function SubmitUrl() {
  const [url, setUrl] = useState("");
  const [notice, setNotice] = useState(null);
  const [error, setError] = useState(null);

  const { commit, inFlight } = useMutation(graphql`
    mutation SubmitUrl($url: String!) {
      submitUrl(input: { url: $url }) {
        id
        title
      }
    }
  `, {
    onCommit: ({ submitUrl: { id, title } }) => {
      setUrl("");
      setError(null);
      if (title) {
        setNotice(`"${title}" was successfully submitted`);
      } else {
        setNotice("Your url was successfully submitted");
      }
    },
    onError: ([{message}]) => {
      setNotice(null);
      setError(`Failed to submit: ${message}`);
    },
  });

  const submit = e => {
    e.preventDefault();
    commit({ url: url.trim() });
  };

  return (
    <form class="w-full" onSubmit={submit}>
      {notice && <Notice message={notice} style="mb-2" />}
      {error && <Notice message={error} type="error" style="mb-2" />}
      <TextInput
        label="URL to something interesting"
        placeholder="https://urls.fyi"
        value={url}
        onChange={setUrl}
      />
      <Button
        title="Submit"
        style="w-full mt-2"
        loading={inFlight}
        disabled={inFlight}
      />
    </form>
  );
}
