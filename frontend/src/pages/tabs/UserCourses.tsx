import React, { FC } from 'react';
import { Typography, Box } from '@mui/material';

const UserCourses: FC = () => {
  return (
    <Box>
      <Typography variant="h6" component="h2">
        Your Courses
      </Typography>
      <Typography variant="body1">
        Here is the list of courses you are enrolled in.
      </Typography>
    </Box>
  );
};

export default UserCourses;
