
pub mod  solution{

    pub fn three_integer_sum(nums_list: &mut[i32])
    {
        nums_list.sort();
        let mut i = 0;
        let mut res = vec![];
        while i < nums_list.len()
        {
            let mut left: usize = i + 1;
            let mut right = nums_list.len() - 1;
            while left < right {
                let current_sum = nums_list[i] + nums_list[left] + nums_list[right] ;
                println!("{}", current_sum);
                if current_sum < 0 {
                    println!("hello left");
                    left += 1;
                }
                else if current_sum > 0{
                    println!("hello right");
                    right -= 1;
                }
                else if current_sum == 0  {
                    if nums_list[left] != nums_list[left - 1] && nums_list[right] != nums_list[right - 1]{
                        res.push(vec![nums_list[i], nums_list[left], nums_list[right]]);
                    }
                    left += 1;
                }
                println!("hello night");
            }
            println!("break");
            i+=1;
        }

    }
}

fn main()
{
    let mut nums_list = vec![-1,0,1,2,-1,-4];
    println!("{:?}", solution::three_integer_sum(&mut nums_list));
}