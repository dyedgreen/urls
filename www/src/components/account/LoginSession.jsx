import { Fragment, h } from "preact";
import { useState } from "preact/hooks";
import { graphql, useMutation } from "picoql";

import Notice from "@app/Notice";
import Button from "@app/Button";

const FINGER_PRINT_ICON = (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    class="h-5 w-5"
    viewBox="0 0 20 20"
    fill="currentColor"
  >
    <path
      fill-rule="evenodd"
      d="M6.625 2.655A9 9 0 0119 11a1 1 0 11-2 0 7 7 0 00-9.625-6.492 1 1 0 11-.75-1.853zM4.662 4.959A1 1 0 014.75 6.37 6.97 6.97 0 003 11a1 1 0 11-2 0 8.97 8.97 0 012.25-5.953 1 1 0 011.412-.088z"
      clip-rule="evenodd"
    />
    <path
      fill-rule="evenodd"
      d="M5 11a5 5 0 1110 0 1 1 0 11-2 0 3 3 0 10-6 0c0 1.677-.345 3.276-.968 4.729a1 1 0 11-1.838-.789A9.964 9.964 0 005 11zm8.921 2.012a1 1 0 01.831 1.145 19.86 19.86 0 01-.545 2.436 1 1 0 11-1.92-.558c.207-.713.371-1.445.49-2.192a1 1 0 011.144-.83z"
      clip-rule="evenodd"
    />
    <path
      fill-rule="evenodd"
      d="M10 10a1 1 0 011 1c0 2.236-.46 4.368-1.29 6.304a1 1 0 01-1.838-.789A13.952 13.952 0 009 11a1 1 0 011-1z"
      clip-rule="evenodd"
    />
  </svg>
);

export default function Invite({ id, lastUsed, lastUserAgent, onRevoke }) {
  const date = new Date(lastUsed);

  const [error, setError] = useState(null);
  const { commit, inFlight } = useMutation(
    graphql`
    mutation RevokeLogin($login: ID!) {
      revokeLogin(login: $login) {
        ok
      }
    }
  `,
    {
      onCommit: onRevoke,
      onError: ([{ message }]) => setError(message),
    },
  );

  return (
    <>
      {error && <Notice message={error} type="error" />}
      <div class="w-full p-2 rounded bg-gray-300 dark:bg-gray-600">
        <div class="flex items-start">
          {FINGER_PRINT_ICON}
          <span class="ml-2">
            <h1 class="leading-5 mb-1">
              {lastUserAgent.name} &mdash; {lastUserAgent.operatingSystem}
            </h1>
            <h2 class="leading-none text-gray-500">
              Last used {date.toDateString()} at {date.toTimeString()}
            </h2>
            <Button
              title="Revoke"
              type="flat"
              onClick={() => {
                if (confirm("Are you sure?") === true) {
                  commit({ login: id });
                }
              }}
              loading={inFlight}
              disabled={inFlight}
              style="mt-2"
            />
          </span>
        </div>
      </div>
    </>
  );
}
