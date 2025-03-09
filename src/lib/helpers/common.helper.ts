export const isValidJSON = (str: any): boolean => {
    if (typeof str !== 'string' || str.trim() === '') {
        return false; // Not a valid string
    }

    try {
        JSON.parse(str);
        return true; // The string is valid JSON
    } catch (e) {
        return false; // The string is not valid JSON
    }
}

export default {
    isValidJSON,
}