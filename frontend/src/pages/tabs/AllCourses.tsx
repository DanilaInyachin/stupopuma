import React, { FC } from 'react';
import { Typography, Box, Button } from '@mui/material';
import axios from 'axios';

const AllCourses: FC = () => {
  const handleOnClick = async () => {
    try {
      const response = await axios.get('//localhost:8080/view_courses');
      console.log(response.data);
    } catch (error) {
      console.error('Error: ', error);
    }
  };

  return (
    <Box>
      <Typography variant="h6" component="h2">
        All Courses
      </Typography>
      <Typography variant="body1">
        Here is the list of all available courses.
      </Typography>
      <Button onClick={handleOnClick}>
        View All Courses
      </Button>
    </Box>
  );
};

export default AllCourses;
