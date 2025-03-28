// TODO: check if this is necessary, no code seems to be using it and parsing JSON we're not gonna use is kinda wasteful anyway
export function isValidJSON(str: any): boolean {
	if (typeof str !== "string" || str.trim() === "") {
		return false; // Not a valid string
	}

	try {
		JSON.parse(str);
		return true; // The string is valid JSON
	} catch (e) {
		return false; // The string is not valid JSON
	}
}
