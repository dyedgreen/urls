import { h } from "preact";

import InfoChip from "@app/InfoChip";

const ICON_USER = <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" viewBox="0 0 20 20" fill="currentColor">
  <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-6-3a2 2 0 11-4 0 2 2 0 014 0zm-2 4a5 5 0 00-4.546 2.916A5.986 5.986 0 0010 16a5.986 5.986 0 004.546-2.084A5 5 0 0010 11z" clip-rule="evenodd" />
</svg>;
const ICON_DATE = <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" viewBox="0 0 20 20" fill="currentColor">
  <path fill-rule="evenodd" d="M6 2a1 1 0 00-1 1v1H4a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2h-1V3a1 1 0 10-2 0v1H7V3a1 1 0 00-1-1zm0 5a1 1 0 000 2h8a1 1 0 100-2H6z" clip-rule="evenodd" />
</svg>;

function formatDate(dateStr) {
  const days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
  const month = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
  const date = new Date(dateStr);
  return `${days[date.getDay()]} ${date.getDate()}. ${month[date.getMonth()]} ${date.getFullYear()}`;
}

export default function Comment({ id, html, createdAt, createdBy }) {
  return (
    <div class="w-full">
      <div dangerouslySetInnerHTML={{ __html: html }} />
      <div
        class="
          flex justify-start items-center
          text-sm italic text-gray-400 dark:text-gray-500
          space-x-1
        "
        >
        <InfoChip icon={ICON_USER} text={createdBy.name} />
        <span class="sm:block hidden">&middot;</span>
        <InfoChip icon={ICON_DATE} text={formatDate(createdAt)} />
      </div>
    </div>
  );
}
