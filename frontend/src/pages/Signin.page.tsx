import {
  Container,
  Grid,
  Box,
  Typography,
  Stack,
  Link as MuiLink,
  FormControlLabel,
  Checkbox,
} from '@mui/material';
import { FC } from 'react';
import { useForm, SubmitHandler, FormProvider } from 'react-hook-form';
import { Link } from 'react-router-dom';
import { zodResolver } from '@hookform/resolvers/zod';
import { literal, object, string, TypeOf } from 'zod';
import FormInput from '../components/FormInput';
import styled from '@emotion/styled';
import { LoadingButton } from '@mui/lab';
import { useTranslation } from 'react-i18next';
import axios from 'axios';

// 👇 Styled React Route Dom Link Component
export const LinkItem = styled(Link)`
  text-decoration: none;
  color: #3683dc;
  &:hover {
    text-decoration: underline;
    color: #5ea1b6;
  }
`;

// 👇 Styled Material UI Link Component
export const OauthMuiLink = styled(MuiLink)`
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #f5f6f7;
  border-radius: 1;
  padding: 0.6rem 0;
  column-gap: 1rem;
  text-decoration: none;
  color: #393e45;
  font-weight: 500;
  cursor: pointer;

  &:hover {
    background-color: #fff;
    box-shadow: 0 1px 13px 0 rgb(0 0 0 / 15%);
  }
`;

const SigninPage: FC = () => {
  const { t } = useTranslation();

  // 👇 Login Schema with Zod
  const loginSchema = object({
    email: string().min(1, t('Email is required')).email(t('Email is invalid')),
    password: string()
      .min(1, t('Password is required'))
      .min(6, t('Password must be more'))
      .max(32, t('Password must be less')),
    persistUser: literal(true).optional(),
  });

  // 👇 Infer the Schema to get the TS Type
  type ILogin = TypeOf<typeof loginSchema>;

  // 👇 Default Values
  const defaultValues: ILogin = {
    email: '',
    password: '',
  };

  // 👇 The object returned from useForm Hook
  const methods = useForm<ILogin>({
    resolver: zodResolver(loginSchema),
    defaultValues,
  });

  // 👇 Submit Handler
  const onSubmitHandler: SubmitHandler<ILogin> = async (values: ILogin) => {
    console.log(values);
    try {
      const response = await axios.post('//localhost:8080/login', {
        mail: values.email,
        password: values.password,
      });
      console.log(response.data);
    } catch (error) {
      console.error('Error: ', error);
    }
  };

  // 👇 JSX to be rendered
  return (
    <Container
      maxWidth={false}
      sx={{ height: '100vh', backgroundColor: { xs: '#fff', md: '#f4f4f4' } }}
    >
      <Grid
        container
        justifyContent="center"
        alignItems="center"
        sx={{ width: '100%', height: '100%' }}
      >
        <Grid
          item
          sx={{ maxWidth: '70rem', width: '100%', backgroundColor: '#fff' }}
        >
          <FormProvider {...methods}>
            <Grid
              container
              sx={{
                boxShadow: { sm: '0 0 5px #ddd' },
                py: '6rem',
                px: '1rem',
              }}
            >
              <Grid
                item
                container
                justifyContent="space-between"
                rowSpacing={5}
                sx={{
                  maxWidth: { sm: '45rem' },
                  marginInline: 'auto',
                }}
              >
                <Grid item xs={12}>
                  <Box
                    display="flex"
                    flexDirection="column"
                    component="form"
                    noValidate
                    autoComplete="off"
                    sx={{ paddingRight: { sm: '3rem' } }}
                    onSubmit={methods.handleSubmit(onSubmitHandler)}
                  >
                    <Typography
                      variant="h6"
                      component="h1"
                      sx={{ textAlign: 'center', mb: '1.5rem' }}
                    >
                      {t('Signin')}
                    </Typography>

                    <FormInput
                      label={t('Enter email')}
                      type="email"
                      name="email"
                      focused
                      required
                    />
                    <FormInput
                      type="password"
                      label={t('Password')}
                      name="password"
                      required
                      focused
                    />

                    <FormControlLabel
                      control={
                        <Checkbox
                          size="small"
                          aria-label="trust this device checkbox"
                          required
                          {...methods.register('persistUser')}
                        />
                      }
                      label={
                        <Typography
                          variant="body2"
                          sx={{
                            fontSize: '0.8rem',
                            fontWeight: 400,
                            color: '#5e5b5d',
                          }}
                        >
                          {t('Trust')}
                        </Typography>
                      }
                    />

                    <LoadingButton
                      loading={false}
                      type="submit"
                      variant="contained"
                      sx={{
                        py: '0.8rem',
                        mt: 2,
                        width: '80%',
                        marginInline: 'auto',
                      }}
                    >
                      {t('Login')}
                    </LoadingButton>
                  </Box>
                </Grid>
              </Grid>
              <Grid container justifyContent="center">
                <Stack sx={{ mt: '3rem', textAlign: 'center' }}>
                  <Typography sx={{ fontSize: '0.9rem', mb: '1rem' }}>
                    {t('Need an account') + ' '}
                    <LinkItem to="/signup">{t('Sign up')}</LinkItem>
                  </Typography>
                </Stack>
              </Grid>
            </Grid>
          </FormProvider>
        </Grid>
      </Grid>
    </Container>
  );
};

export default SigninPage;
