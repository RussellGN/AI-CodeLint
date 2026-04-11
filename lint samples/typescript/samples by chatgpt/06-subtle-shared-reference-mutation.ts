type Profile = { name: string; tags: string[] };

const baseProfile: Profile = { name: "Guest", tags: ["new"] };
const cachedProfile = baseProfile;

cachedProfile.tags.push("premium");
console.log(baseProfile.tags);
