import { render, h } from "preact";
import { useState } from "preact/hooks";
import { graphql, useMutation } from "picoql";

import ErrorBoundary from "@app/ErrorBoundary";
import Button from "@app/Button";
import Link from "@app/Link";
import SubmitUrl from "@app/header/SubmitUrl";


function Header() {
  const [showSubmit, setShowSubmit] = useState(false);

  let containerClasses = "w-full rounded-t-lg pb-8 -mb-6 p-2";
  if (showSubmit)
    containerClasses += " bg-gray-50 dark:bg-gray-900";

  return (
    <div class={containerClasses}>
      <div class="flex space-x-2 items-center">
        <Button title="Submit a url" onClick={() => setShowSubmit(!showSubmit)} />
        <Link title="/recent" href="/recent" />
        <Link title="/mine" href="/mine" />
      </div>
      <div class="mt-4" style={{ display: showSubmit ? undefined : "none" }}>
        <SubmitUrl />
      </div>
    </div>
  );
}

function VoteButton({ urlID, initDidVote, initCount }) {
  const [didVote, setDidVote] = useState(initDidVote ?? false);
  const [count, setCount] = useState(initCount ?? 0);
  const classes = `
        block w-10 h-10 flex-none m-2 sm:mx-0
        flex flex-col justify-center items-center
        border-2 border-blue-500 rounded
        ${!didVote ? "text-blue-500" : "bg-blue-500 text-white"}
  `;

  const onCommit = ({ url }) => {
    setCount(url?.upvoteCount ?? count);
    setDidVote(url?.upvotedByViewer ?? didVote);
  };

  const upvote = useMutation(graphql`
    mutation Upvote($id: ID!) {
      url: upvoteUrl(url: $id) {
        id
        upvotedByViewer
        upvoteCount
      }
    }
  `, { onCommit, onError: () => {}, });

  const rescind = useMutation(graphql`
    mutation Upvote($id: ID!) {
      url: rescindUrlUpvote(url: $id) {
        id
        upvotedByViewer
        upvoteCount
      }
    }
  `, { onCommit, onError: () => {}, });

  const click = e => {
    e.preventDefault();
    if (didVote) {
      setCount(count - 1);
      setDidVote(false);
      rescind.commit({ id: urlID });
    } else {
      setCount(count + 1);
      setDidVote(true);
      upvote.commit({ id: urlID });
    }
  };

  return (
    <button class={classes} onClick={click}>
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M5.293 9.707a1 1 0 010-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 01-1.414 1.414L11 7.414V15a1 1 0 11-2 0V7.414L6.707 9.707a1 1 0 01-1.414 0z" clip-rule="evenodd" />
      </svg>
      <span class="text-xs leading-none">{count}</span>
    </button>
  );
}

render(<ErrorBoundary><Header /></ErrorBoundary>, document.getElementById("header"));
for (const element of document.querySelectorAll("[data-hydrate-vote-button]")) {
  const urlID = element.dataset.id;
  const count = parseInt(element.dataset.count);
  const upvoted = element.dataset.upvoted === "true";
  render(<VoteButton urlID={urlID} initDidVote={upvoted} initCount={count} />, element);
}