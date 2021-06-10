const lists = [
  "https://raw.githubusercontent.com/disposable/disposable-email-domains/master/domains.txt",
  "https://raw.githubusercontent.com/wesbos/burner-email-providers/master/emails.txt",
];

const allHosts = new Set();
for (const listUrl of lists) {
  const list = await fetch(listUrl).then(resp => resp.text());
  for (const host of list.split("\n")) {
    if (host.trim().length > 0)
      allHosts.add(host.trim());
  }
}

const allHostsTxt = [...allHosts].join("\n");
await Deno.writeTextFile("domains.txt", allHostsTxt);
