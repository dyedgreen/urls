import { render, h } from "preact";
import { useState } from "preact/hooks";

import ErrorBoundary from "@app/ErrorBoundary";
import Button from "@app/Button";
import Link from "@app/Link";
import SubmitUrl from "@app/header/SubmitUrl";

function Header() {
  const [showSubmit, setShowSubmit] = useState(false);

  let containerClasses = "w-full rounded-t-lg pb-8 -mb-6 p-2";
  if (showSubmit)
    containerClasses += " bg-gray-300 dark:bg-gray-900";

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

render(<ErrorBoundary><Header /></ErrorBoundary>, document.getElementById("header"));
