import { h, Fragment } from "preact";
import { useState } from "preact/hooks";
import { graphql, useMutation } from "picoql";

import Button from "@app/Button";
import Notice from "@app/Notice";
import Comment from "@app/comments/Comment";

const ICON_SEND = <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
  <path d="M10.894 2.553a1 1 0 00-1.788 0l-7 14a1 1 0 001.169 1.409l5-1.429A1 1 0 009 15.571V11a1 1 0 112 0v4.571a1 1 0 00.725.962l5 1.428a1 1 0 001.17-1.408l-7-14z" />
</svg>;

export default function CommentInput({ urlID, repliesToID }) {
  const [text, setText] = useState("");
  const [error, setError] = useState(null);

  const [newComments, setNewComments] = useState([]);

  const { commit, inFlight } = useMutation(graphql`
    mutation CommentInputMutation($text: String!, $url: ID!) {
      comment(input: { comment: $text, url: $url }) {
        ...CommentFragment
      }
    }

    fragment CommentFragment on Comment {
      id
      html
      createdAt
      createdBy {
        id
        name
      }
    }
  `, {
    onCommit: ({ comment }) => {
      setText("");
      setError(null);
      setNewComments([...newComments, comment]);
    },
    onError: ([{ message }]) => {
      setError(message);
    },
  });

  const disabled = text.trim().length == 0 || inFlight;
  const submit = e => {
    e.preventDefault();
    if (disabled)
      return;
    commit({ text: text.trim(), url: urlID });
  };

  return (
    <>
      {newComments.map(comment => <Comment {...comment} />)}
      <form class="w-full" onSubmit={submit}>
        {error && <Notice message={error} type="error" style="mb-2" />}
        <div class="w-full p-2 rounded bg-gray-200 dark:bg-gray-600">
          <textarea
            class="w-full h-14 resize-none bg-transparent leading-none"
            placeholder="Your thoughts, formatted with *markdown* ..."
            onInput={e => setText(e.target.value)}
            value={text}
          >{text}</textarea>
          <Button
            title={<div class="flex items-center">{ICON_SEND} Comment</div>}
            onClick={submit}
            disabled={disabled}
            loading={inFlight}
          />
        </div>
      </form>
    </>
  );
}
