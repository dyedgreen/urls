import { h } from "preact";

const ICON = <svg xmlns="http://www.w3.org/2000/svg" class="h-10 w-10" fill="none" viewBox="0 0 24 24" stroke="currentColor">
  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 5v2m0 4v2m0 4v2M5 5a2 2 0 00-2 2v3a2 2 0 110 4v3a2 2 0 002 2h14a2 2 0 002-2v-3a2 2 0 110-4V7a2 2 0 00-2-2H5z" />
</svg>;

export default function Invite({ id, token, claimedBy }) {
  return (
    <div class="w-full mb-2 rounded-xl overflow-hidden">
      <div
        class="
          bg-yellow-400 text-yellow-900
          divide-dashed divide-y divide-yellow-600 divide-y-2
        "
      >
        <div class="flex p-4 items-center">
          {ICON}
          <h1 class="ml-4 text-xl font-semibold">
            Account Registration Code <span class="text-yellow-600">admits one</span>
          </h1>
        </div>
        {
          claimedBy == null ?
          <div class="p-4">
            <pre class="w-full text-center p-2 bg-yellow-500 rounded-md">
              {token}
            </pre>
          </div> : <div />
        }
      </div>
      {
        claimedBy != null &&
        <p class="p-4 font-semibold text-center text-gray-500">
          Claimed by {claimedBy?.name}
        </p>
      }
    </div>
  );
}
