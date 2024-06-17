import React, { FC, useContext, useEffect, useState } from 'react';
import { Typography, Box } from '@mui/material';
import axios from 'axios';
import ElementCourse from '../../components/ElementCourse';
import { useNavigate } from 'react-router-dom';
import CurrentUserContext from '../../context';

interface AllCourseProps {
  needButton?: boolean;
}

const AllCourses: FC<AllCourseProps> = ({ needButton = true }) => {
  const [namecourses, setNamecourses] = useState<string[]>([]);
  const navigate = useNavigate();
  const context = useContext(CurrentUserContext);

  useEffect(() => {
    if (!context || !context.isAuthAndToken) {
      navigate('/signin');
    } else {
      axios
        .get('//localhost:8080/view_courses')
        .then((response) => {
          console.log(response);
          setNamecourses(response.data);
        })
        .catch((error) => {
          console.error('Error: ', error);
        });
    }
  }, [context, navigate, setNamecourses]);

  return (
    <Box>
      <Typography variant="h6" component="h2">
        All Courses
      </Typography>
      <Typography variant="body1">
        Here is the list of all available courses.
      </Typography>
      {namecourses.map((course) => (
        <ElementCourse
          key={course}
          namecourse={course}
          needButton={needButton}
        />
      ))}
    </Box>
  );
};

export default AllCourses;
