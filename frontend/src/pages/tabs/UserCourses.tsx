import React, { FC, useContext, useEffect, useState } from 'react';
import { Typography, Box, Divider } from '@mui/material';
import { useNavigate } from 'react-router-dom';
import CurrentUserContext from '../../context';
import axios from 'axios';
import ElementCourse from '../../components/ElementCourse';
import { useTranslation } from 'react-i18next';

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
  const { t } = useTranslation();

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
        {t('My courses')}
      </Typography>
      {userCourses.enrolled_courses.map((course) => (
        <ElementCourse key={course} namecourse={course} needButton={false} />
      ))}
      <Divider />
      <Typography variant="h6" component="h2">
        {t('Under consideration')}
      </Typography>
      {userCourses.not_enrolled_courses.map((course) => (
        <ElementCourse key={course} namecourse={course} needButton={false} />
      ))}
    </Box>
  );
};

export default UserCourses;
