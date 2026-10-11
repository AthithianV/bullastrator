const urlRegex = /(https?:\/\/[^\s]+)/g;

export const extractLink = (text: string) => {
  return text.split(urlRegex);
};

export const isLink = (part: string) => urlRegex.test(part);
