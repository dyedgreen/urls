import { h, render } from "preact";
import { graphql, useQuery } from "picoql";

import ErrorBoundary from "@app/ErrorBoundary";
import Comment from "@app/comments/Comment";
import ActivityIndicator from "@app/ActivityIndicator";
import CommentInput from "@app/comments/CommentInput";

function Comments({ urlID }) {
  const { data, loading } = useQuery(
    graphql`
    query CommentsQuery($url: ID!) {
      viewer {
        user {
          id
        }
      }
      url: fetch__Url(id: $url) {
        id
        comments {
          nodes {
            ...CommentFragment
          }
        }
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
  `,
    { url: urlID },
  );
  return (
    <div
      class="w-full flex flex-col items-center justify-center space-y-1 sm:pl-14"
    >
      {(data?.url?.comments?.nodes?.length ?? 0) > 0 &&
        <h2 class="w-full text-xl font-semibold">Comments</h2>}
      {loading
        ? <ActivityIndicator size="large" />
        : data?.url?.comments?.nodes?.map((node) => <Comment {...node} />)}
      {data?.viewer?.user?.id && <CommentInput urlID={urlID} />}
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
