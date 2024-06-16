import { Box, Container, Grid, Tab, Tabs, Typography } from '@mui/material';
import { FC, useState } from 'react';
import { useTranslation } from 'react-i18next';
import GeneralInfo from './tabs/GeneralInfo';
import AllCourses from './tabs/AllCourses';
import UserCourses from './tabs/UserCourses';
import styled from '@emotion/styled';

const VerticalTabs = styled(Tabs)({
  borderRight: `1px solid #ddd`,
});

const UserProfilePage: FC = () => {
  const { t } = useTranslation();
  const [value, setValue] = useState(0);

  const handleChange = (event: React.SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };

  return (
    <Container maxWidth="lg">
      <Typography variant="h4" component="h1" sx={{ textAlign: 'center', mt: 4 }}>
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
            <Tab label={t('General info')} />
            <Tab label={t('All courses')} />
            <Tab label={t('Followed courses')} />
          </VerticalTabs>
        </Grid>
        <Grid item xs={9}>
          <Box sx={{ p: 3 }}>
            {value === 0 && <GeneralInfo />}
            {value === 1 && <AllCourses />}
            {value === 2 && <UserCourses />}
          </Box>
        </Grid>
      </Grid>
    </Container>
  );
};

export default UserProfilePage;
