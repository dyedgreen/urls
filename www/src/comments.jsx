import { h, render } from "preact";

import ErrorBoundary from "@app/ErrorBoundary";
import CommentInput from "@app/comments/CommentInput";

function Comments({ urlID }) {
  return (
    <div class="w-full flex flex-col items-center justify-center space-y-1 -mt-3 sm:pl-14">
      <CommentInput urlID={urlID} />
    </div>
  );
}

const commentsElement = document.getElementById("comments");
render(
  <ErrorBoundary>
    <Comments urlID={commentsElement.dataset.urlId} />
  </ErrorBoundary>,
  commentsElement,
);
