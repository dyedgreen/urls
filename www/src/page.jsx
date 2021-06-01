import { render, h, Fragment } from "preact";
import { useState } from "preact/hooks";

import ErrorBoundary from "@app/ErrorBoundary";
import Button from "@app/Button";
import Link from "@app/Link";
import SubmitUrl from "@app/header/SubmitUrl";

function VoteButton({ initDidVote, initCount }) {
  const [didVote, setDidVote] = useState(initDidVote ?? false);
  const [count, setCount] = useState(initCount ?? 0);
  const classes = `
        block w-10 h-10 flex-none m-2 mr-0
        flex flex-col justify-center items-center
        border-2 border-blue-500 rounded
        ${!didVote ? "text-blue-500" : "bg-blue-500 text-white"}
  `;

  const click = e => {
    e.preventDefault();
    if (didVote) {
      setCount(count - 1);
      setDidVote(false);
    } else {
      setCount(count + 1);
      setDidVote(true);
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

function Header() {
  const [showSubmit, setShowSubmit] = useState(false);

  let containerClasses = "w-full rounded-t-lg pb-8 -mb-6 p-2";
  if (showSubmit)
    containerClasses += " bg-gray-50 dark:bg-gray-900";

  const test = [
    { title: "Test title here", description: "Your description here.", url: "https://tilman.dev", image: "https://opengraph.githubassets.com/fc650ffe80c1f30447363840b1e7531d7ef69c492576ba8c0dbf9c9ac2c31ae8/dyedgreen/deno-sqlite" },
    { title: "Test title here", didVote: true, description: "Your description here.", url: "https://tilman.dev" },
    { title: "The Accidental Rush for Anthrax Island", description: "Gruinard Island, in the north-west of Scotland, was where Britain tested its biological weapons. That story&#39;s been told many times: but I found something in ...", url: "https://tilman.dev", image: "https://i.ytimg.com/vi/suAC_PDP3Sk/maxresdefault.jpg" },
    { title: "Models of Generics and Metaprogramming: Go, Rust, Swift, D and More", description: "Your description here.", url: "https://tilman.dev", image: "https://opengraph.githubassets.com/fc650ffe80c1f30447363840b1e7531d7ef69c492576ba8c0dbf9c9ac2c31ae8/dyedgreen/deno-sqlite" },
  ];

  return (
    <>
    <div
      class="
        w-full max-w-screen-md p-4 mb-8
        space-y-4
        bg-white dark:bg-gray-800 shadow rounded-lg
      "
    >
        {test.map(({title, description, image, url, didVote }) => {
          return (
          <a class="block w-full sm:flex rounded hover:bg-gray-200 dark:hover:bg-gray-700" href={url}>
            <VoteButton initDidVote={didVote} initCount={42} />
            <div class="sm:flex-grow p-2">
              <h1 class="leading-5 text-xl font-semibold">{title}</h1>
              <p class="mt-1 leading-4 text-sm text-gray-600 dark:text-gray-500">
                <span class="text-gray-400 underline italic">{url}</span> &middot; {description}
              </p>
              <div
                class="
                  flex items-center mt-1
                  italic leading-4 text-sm text-gray-400 dark:text-gray-500
                "
              >
                <a class="block p-1 -ml-1 rounded-xl flex items-center hover:bg-gray-300" href="/user/...">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-6-3a2 2 0 11-4 0 2 2 0 014 0zm-2 4a5 5 0 00-4.546 2.916A5.986 5.986 0 0010 16a5.986 5.986 0 004.546-2.084A5 5 0 0010 11z" clip-rule="evenodd" />
                  </svg>
                  Peter Parker
                </a>
                &middot;
                <a class="block p-1 rounded-xl flex items-center hover:bg-gray-300" href="/user/...">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" viewBox="0 0 20 20" fill="currentColor">
                    <path d="M2 5a2 2 0 012-2h7a2 2 0 012 2v4a2 2 0 01-2 2H9l-3 3v-3H4a2 2 0 01-2-2V5z" />
                    <path d="M15 7v2a4 4 0 01-4 4H9.828l-1.766 1.767c.28.149.599.233.938.233h2l3 3v-3h2a2 2 0 002-2V9a2 2 0 00-2-2h-1z" />
                  </svg>
                  5 Comments
                </a>
                &middot;
                Jun. 24. 2021
              </div>
            </div>
            {image && <img class="w-full sm:w-40 sm:h-20 max-w-full rounded shadow-xl" src={image} />}
          </a>
          );
        })}
    </div>
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
    </>
  );
}

render(<ErrorBoundary><Header /></ErrorBoundary>, document.getElementById("header"));
