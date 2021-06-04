import { render, h, Component } from "preact";
import { useState } from "preact/hooks";
import { graphql, useMutation } from "picoql";
import { errorToString } from "@app/ErrorBoundary";

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

class VoteErrorBoundary extends Component {
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
      return (
        <div
          class="block w-10 h-10 flex-none m-2 sm:mx-0 flex justify-center items-center rounded bg-red-500 text-white"
          title={errorToString(error)}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
          </svg>
        </div>
      );
    }
    return children;
  }
}

export default function hydrate() {
  for (const element of document.querySelectorAll("[data-hydrate-vote-button]")) {
    const urlID = element.dataset.id;
    const count = parseInt(element.dataset.count);
    const upvoted = element.dataset.upvoted === "true";
    const button = (
      <VoteErrorBoundary>
        <VoteButton urlID={urlID} initDidVote={upvoted} initCount={count} />
      </VoteErrorBoundary>
    );
    render(button, element);
  }
}
