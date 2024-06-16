import { CssBaseline, ThemeProvider, createTheme } from '@mui/material';
import React, {
  Dispatch,
  SetStateAction,
  createContext,
  useEffect,
  useState,
} from 'react';
import { Route, Routes } from 'react-router-dom';
import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import SigninPage from './pages/Signin.page';
import SignupPage from './pages/Signup.page';
import UserProfilePage from './pages/UserProfile.page';
import CurrentUserContext from './contex';

const theme = createTheme();

function App() {
  const [isI18NInitialized, setIsI18NInitialized] = useState<boolean>(false);
  const [lang] = useState<string>('ru');
  const [isAuthAndToken, setIsAuthAndToken] = useState<string | undefined>(
    undefined
  );

  useEffect(() => {
    i18n
      .use(initReactI18next)
      .init({
        lng: lang,
        resources: require(`./i18n/${lang}.json`),
        fallbackLng: lang,
      })
      .then(() => {
        setIsI18NInitialized(true);
      });
  }, [lang]);

  if (!isI18NInitialized) {
    return <div>Loading...</div>;
  }

  return (
    <div className="App">
      <ThemeProvider theme={theme}>
        <CurrentUserContext.Provider
          value={{ isAuthAndToken, setIsAuthAndToken }}
        >
          <CssBaseline />
          <Routes>
            <Route path="/" element={<SigninPage />} />
            <Route path="/signin" element={<SigninPage />} />
            <Route path="/signup" element={<SignupPage />} />
            <Route path="/profile" element={<UserProfilePage />} />
          </Routes>
        </CurrentUserContext.Provider>
      </ThemeProvider>
    </div>
  );
}

export default App;
