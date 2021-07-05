import { render, h, Fragment } from "preact";
import { useState } from "preact/hooks";
import hydrateUpvotes from "./upvote.jsx";

import ErrorBoundary from "@app/ErrorBoundary";
import Button from "@app/Button";
import Link from "@app/Link";
import SubmitUrl from "@app/header/SubmitUrl";

const MENU_ITEM = <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
  <path fill-rule="evenodd" d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM9 15a1 1 0 011-1h6a1 1 0 110 2h-6a1 1 0 01-1-1z" clip-rule="evenodd" />
</svg>;
const MENU_SEARCH = <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
  <path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd" />
</svg>;

function Header() {
  const [showSubmit, setShowSubmit] = useState(false);
  const [showMenu, setShowMenu] = useState(false);

  let containerClasses = "w-full rounded-t-lg pb-8 -mb-6 p-2";
  if (showSubmit || showMenu)
    containerClasses += " bg-gray-50 dark:bg-gray-900";

  const links = (
    <>
      <Link title="home" href="/" />
      <Link title="recent" href="/recent" />
      <Link title="best" href="/best" />
      <Link title="mine" href="/mine" />
    </>
  );

  const toggleSubmit = () => {
    setShowSubmit(!showSubmit);
    setShowMenu(false);
  };
  const toggleMenu = () => {
    setShowMenu(!showMenu);
    setShowSubmit(false);
  };

  return (
    <div class={containerClasses}>
      <div class="flex items-center justify-between">
        <Button title="Submit url" onClick={toggleSubmit} style="mr-2 whitespace-nowrap" />
        <div class="w-full sm:flex hidden">
          {links}
        </div>
        <div class="flex items-center space-x-2">
          <Link title={MENU_SEARCH} href="/search" type="flat" style="block" />
          <Button title={MENU_ITEM} onClick={toggleMenu} type="flat" style="sm:hidden block" />
        </div>
      </div>
      <div class="mt-4" style={{ display: showSubmit ? undefined : "none" }}>
        <SubmitUrl />
      </div>
      {showMenu && <div class="mt-4">{links}</div>}
    </div>
  );
}

render(<ErrorBoundary><Header /></ErrorBoundary>, document.getElementById("header"));
hydrateUpvotes();
