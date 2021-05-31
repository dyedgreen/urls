import { render, h, Fragment } from "preact";
import { useState } from "preact/hooks";

import ErrorBoundary from "@app/ErrorBoundary";
import Button from "@app/Button";
import Link from "@app/Link";
import SubmitUrl from "@app/header/SubmitUrl";

function Header() {
  const [showSubmit, setShowSubmit] = useState(false);

  let containerClasses = "w-full rounded-t-lg pb-8 -mb-6 p-2";
  if (showSubmit)
    containerClasses += " bg-gray-50 dark:bg-gray-900";

  const test = [
    { title: "Test title here", description: "Your description here.", url: "https://tilman.dev", image: "https://opengraph.githubassets.com/fc650ffe80c1f30447363840b1e7531d7ef69c492576ba8c0dbf9c9ac2c31ae8/dyedgreen/deno-sqlite" },
    { title: "Test title here", description: "Your description here.", url: "https://tilman.dev" },
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
        {test.map(({title, description, image, url}) => {
          return (
          <div class="w-full sm:flex rounded hover:bg-gray-200">
            <a class="block sm:flex-grow p-2" href={url}>
              <h1 class="leading-5 text-xl font-semibold">{title}</h1>
              <p class="leading-4 text-sm text-gray-600">{description}</p>
            </a>
            {image && <img class="w-full sm:w-40 sm:h-20 max-w-full rounded shadow-xl" src={image} />}
          </div>
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
