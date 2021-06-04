import { render, h } from "preact";
import hydrateUpvotes from "./upvote.jsx";

import ErrorBoundary from "@app/ErrorBoundary";
import Notice from "@app/Notice";

function Comments() {
  return (
    <div class="w-full flex justify-center">
      <Notice message="Comments are under construction" type="warning" style="w-full" />
    </div>
  );
}

render(<ErrorBoundary><Comments /></ErrorBoundary>, document.getElementById("comments"));
hydrateUpvotes();
