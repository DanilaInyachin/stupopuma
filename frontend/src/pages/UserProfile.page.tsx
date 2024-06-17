import { Box, Container, Grid, Tab, Tabs, Typography } from '@mui/material';
import { FC, useContext, useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import GeneralInfo from './tabs/GeneralInfo';
import AllCourses from './tabs/AllCourses';
import UserCourses from './tabs/UserCourses';
import styled from '@emotion/styled';
import { useNavigate } from 'react-router-dom';
import CurrentUserContext from '../context';
import axios from 'axios';
import Requests from './tabs/Requests';

const VerticalTabs = styled(Tabs)({
  borderRight: `1px solid #ddd`,
});

type Role = 'Администратор' | 'Преподаватель' | 'Ученик';

const UserProfilePage: FC = () => {
  const getTabs = (role: Role) => {
    switch (role) {
      case 'Ученик':
        return [
          { label: t('General info'), component: <GeneralInfo /> },
          { label: t('All courses'), component: <AllCourses /> },
          { label: t('Followed courses'), component: <UserCourses /> },
        ];
      case 'Администратор':
        return [
          { label: t('General info'), component: <GeneralInfo /> },
          {
            label: t('All courses'),
            component: <AllCourses needButton={false} />,
          },
          { label: t('Requests'), component: <Requests /> },
        ];
      case 'Преподаватель':
        return [
          { label: t('General info'), component: <GeneralInfo /> },
          {
            label: t('All courses'),
            component: <AllCourses needButton={false} />,
          },
        ];
      default:
        return [
          { label: t('General info'), component: <GeneralInfo /> },
          { label: t('All courses'), component: <AllCourses /> },
        ];
    }
  };

  const { t } = useTranslation();
  const [value, setValue] = useState(0);
  const [role, setRole] = useState<Role>('Ученик');
  const [tabs, setTabs] = useState(getTabs(role));
  const navigate = useNavigate();
  const context = useContext(CurrentUserContext);

  useEffect(() => {
    if (!context || !context.isAuthAndToken) {
      navigate('/signin');
    } else {
      axios
        .post('//localhost:8080/view_user', {
          token: context.isAuthAndToken,
        })
        .then((response) => {
          console.log(response);
          setRole(response.data.role);
          setTabs(getTabs(response.data.role));
        })
        .catch((error) => {
          console.error('Error: ', error);
        });
    }
  }, [context, navigate, setRole, setTabs, getTabs]);

  const handleChange = (event: React.SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };

  return (
    <Container maxWidth="lg">
      <Typography
        variant="h4"
        component="h1"
        sx={{ textAlign: 'center', mt: 4 }}
      >
        {t('User profile')}
      </Typography>
      <Grid container spacing={2} sx={{ mt: 2 }}>
        <Grid item xs={3}>
          <VerticalTabs
            orientation="vertical"
            value={value}
            onChange={handleChange}
            aria-label="profile tabs"
            variant="scrollable"
            scrollButtons="auto"
          >
            {tabs.map((tab, index) => (
              <Tab key={index} label={tab.label} />
            ))}
          </VerticalTabs>
        </Grid>
        <Grid item xs={9}>
          <Box sx={{ p: 3 }}>{tabs[value].component}</Box>
        </Grid>
      </Grid>
    </Container>
  );
};

export default UserProfilePage;
