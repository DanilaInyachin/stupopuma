import { Dispatch, SetStateAction, createContext } from "react";

// 👇 Define Context Type
export interface CurrentUserContextType {
    isAuthAndToken: string | undefined;
    setIsAuthAndToken: Dispatch<SetStateAction<string | undefined>>;
}

const CurrentUserContext = createContext<CurrentUserContextType | undefined>(undefined);

export default CurrentUserContext;
