import React, { FC, useContext, useEffect, useState } from 'react';
import { Typography, Box, Button } from '@mui/material';
import axios from 'axios';
import ElementCourse from '../../components/ElementCourse';
import { useNavigate } from 'react-router-dom';
import CurrentUserContext from '../../context';

type Role = 'Администратор' | 'Преподаватель' | 'Ученик';

interface AllCourseProps {
  needButton?: boolean;
}

const AllCourses: FC<AllCourseProps> = ({ needButton = true }) => {
  const [namecourses, setNamecourses] = useState<string[]>([]);
  const navigate = useNavigate();
  const context = useContext(CurrentUserContext);
  const [role, setRole] = useState<Role>('Ученик');

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
        })
        .catch((error) => {
          console.error('Error: ', error);
        });
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
          isTeacher={role === 'Преподаватель'}
        />
      ))}
      {role === 'Преподаватель' && (
        <Button variant="contained" color="primary" sx={{ mt: 2 }}>
          Добавить новый курс
        </Button>
      )}
    </Box>
  );
};

export default AllCourses;
