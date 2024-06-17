import React, { FC, useContext, useEffect, useState } from 'react';
import { Typography, Box, Divider } from '@mui/material';
import { useNavigate } from 'react-router-dom';
import CurrentUserContext from '../../context';
import axios from 'axios';
import ElementCourse from '../../components/ElementCourse';

interface IUserCourses {
  enrolled_courses: string[];
  not_enrolled_courses: string[];
}

const initailState: IUserCourses = {
  enrolled_courses: [],
  not_enrolled_courses: [],
};

const UserCourses: FC = () => {
  const [userCourses, setuserCourses] = useState<IUserCourses>(initailState);
  const navigate = useNavigate();
  const context = useContext(CurrentUserContext);

  useEffect(() => {
    if (!context || !context.isAuthAndToken) {
      navigate('/signin');
    } else {
      axios
        .post('//localhost:8080/get_user_courses_list', {
          token: context.isAuthAndToken,
        })
        .then((response) => {
          console.log(response);
          setuserCourses(response.data);
        })
        .catch((error) => {
          console.error('Error: ', error);
        });
    }
  }, [context, navigate, setuserCourses]);

  return (
    <Box>
      <Typography variant="h6" component="h2">
        Your Courses
      </Typography>
      <Typography variant="body1">
        Here is the list of courses you are enrolled in.
      </Typography>
      <Divider />
      <Typography variant="h6" component="h2">
        Мои курсы
      </Typography>
      {userCourses.enrolled_courses.map((course) => (
        <ElementCourse key={course} namecourse={course} needButton={false} />
      ))}
      <Divider />
      <Typography variant="h6" component="h2">
        На рассмотрении
      </Typography>
      {userCourses.not_enrolled_courses.map((course) => (
        <ElementCourse key={course} namecourse={course} needButton={false} />
      ))}
    </Box>
  );
};

export default UserCourses;
